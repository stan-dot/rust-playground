use tokio::{
    io::{AsyncBufReadExt,AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel(10);
    loop{
      let (mut socket, addr) = listener.accept().await.unwrap();

      let tx = tx.clone();

      let mut rx = tx.subscribe();

      tokio::spawn(async move{
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();


        loop{

        // v1
        //   let mut buffer = [0u8; 1024];
        //   let bytes_read:usize  = socket.read(&mut buffer).await.unwrap();
        //   socket.write_all(&buffer[..bytes_read] ).await.unwrap();

        // v2 this appends lines to the response
        // let bytes_read:usize  = reader.read_line(&mut line).await.unwrap();

        // if bytes_read == 0{
        //   break;
        // }
        // tx.send(line.clone()).unwrap();
        // let msg = rx.recv().await.unwrap();
        // writer.write_all(msg.as_bytes() ).await.unwrap();
        // line.clear();

            tokio::select!{
                result = reader.read_line(&mut line)=>{
                    if result.unwrap() == 0 {
                        break;
                    }
                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                }
                result = rx.recv()=>{
                    let (msg, other_addr) = result.unwrap();
                    if addr != other_addr{
                      writer.write_all(msg.as_bytes() ).await.unwrap();
                    }
                }
            }
        };
      });
    };
}
