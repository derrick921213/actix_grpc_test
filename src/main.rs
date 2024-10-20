mod grpc;
mod rest;

use actix_web::{App, HttpServer};

use grpc::grpc_service;
use tokio::sync::oneshot;
use tonic::transport::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = oneshot::channel();
    let grpc_handle = tokio::spawn(async move {
        let addr = "[::]:50051".parse().unwrap();
        println!("gRPC server listening on {}", addr);
        Server::builder()
            .add_service(grpc_service())
            .serve_with_shutdown(addr, async {
                rx.await.ok();
            })
            .await
            .unwrap();
    });
    let addr = "[::]:8080";
    println!("HTTP server listening on {}", addr);
    let server = HttpServer::new(|| App::new().service(rest::rest_service()))
        .bind(addr)?
        .run();

    let _ = tokio::select! {
        result = server => {
            let _ = tx.send(());
            if let Err(e) = &result {
                eprintln!("Actix Web stopped with an error: {}", e);
            }
            result
        },
        _ = grpc_handle => {
            Ok(())
        }
    };

    Ok(())
}
