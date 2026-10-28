pub async fn shutdown_sig() {
    println!("> Shutdown sig initialized.");
    tokio::signal::ctrl_c()
        .await
        .expect("> Failed to listen to ctrl + c");
    println!("> Received shutdown sig.");
}
