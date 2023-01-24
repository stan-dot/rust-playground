use tokio::{
    io::{AsyncBufReadExt,AsyncWriteExt, BufReader},
    net::TcpListener
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop{
      let (mut socket, _addr) = listener.accept().await.unwrap();

      tokio::spawn(async move{
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop{
        // v1
        //   let mut buffer = [0u8; 1024];
        //   let bytes_read:usize  = socket.read(&mut buffer).await.unwrap();
        //   socket.write_all(&buffer[..bytes_read] ).await.unwrap();

        // v2
            let bytes_read:usize  = reader.read_line(&mut line).await.unwrap();
            // this appends lines to the response

            if bytes_read == 0{
            break;
            }
            writer.write_all(line.as_bytes() ).await.unwrap();
            line.clear();
        };
      });
    };
}
