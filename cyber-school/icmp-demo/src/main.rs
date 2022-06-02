use std::{net::{IpAddr}, time::{Instant, Duration}, sync::{Arc, RwLock}, env};
use pnet::packet::{ip::{IpNextHeaderProtocols,}, icmp::{IcmpTypes, echo_request::{IcmpCodes, MutableEchoRequestPacket}, echo_reply::EchoReplyPacket}, util, Packet};
use pnet_transport::{transport_channel, TransportProtocol};
use pnet_transport::TransportChannelType::Layer4;
use pnet_transport::{icmp_packet_iter};
use rand::random;

const ICMP_SIZE:usize = 64;

fn main() -> anyhow::Result<()>{
    let args: Vec<String> = env::args().collect();
    if(args.len() < 2) {
        panic!("Usage: icmp-demo target_ip");
    }
    let target_ip:IpAddr = args[1].parse().unwrap();
    println!("icpm echo request to target ip:{:#?}",target_ip);

    // 确定协议 并且创建数据包通道 tx 为发送通道, rx 为接收通道
    let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = match transport_channel(4096, protocol) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => return  Err(e.into()),
    };
        
    // 将 rx 接收到的数据包传化为 iterator
    let mut iter = icmp_packet_iter(&mut rx);
    loop {
        let mut icmp_header:[u8;ICMP_SIZE] = [0;ICMP_SIZE];
        let icmp_packet = create_icmp_packet(&mut icmp_header);
        // println!("icmp_packet:{:?}",icmp_packet);
        let timer = Arc::new(RwLock::new(Instant::now()));
        // 发送 ICMP 数据包
        tx.send_to(icmp_packet, target_ip)?;

        match iter.next() {
            // 匹配 EchoReplyPacket 数据包
            Ok((packet, addr)) => match EchoReplyPacket::new(packet.packet()) {
                Some(echo_reply) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        let start_time = timer.read().unwrap();
                        //let identifier = echo_reply.get_identifier();
                        //let sequence_number =  echo_reply.get_sequence_number();
                        let rtt = Instant::now().duration_since(*start_time);
                        println!(
                            "ICMP EchoReply received from {:?}: {:?} , Time:{:?}",
                            addr,
                            packet.get_icmp_type(),
                            rtt
                        );
                    } else {
                        println!(
                            "ICMP type other than reply (0) received from {:?}: {:?}",
                            addr,
                            packet.get_icmp_type()
                        );
                    }
                }
                None => {}
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

/**
 * 创建 icmp EchoRequest 数据包
 */
fn create_icmp_packet<'a>(icmp_header: &'a mut [u8]) -> MutableEchoRequestPacket<'a> {
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    icmp_packet
}
