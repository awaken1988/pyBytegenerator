use std::{time::{Duration}, fs::File, ffi::OsString, io::{Write, Read}, path::Path};

use clap::ArgMatches;
use std::net::UdpSocket;
use crate::protcol::{Opcode,PacketBuilder, TransferMode, Timeout, RECV_TIMEOUT, check_datablock, self, DATA_OFFSET, DEFAULT_BLOCKSIZE, PACKET_SIZE_MAX, PacketParser};


pub fn client_main(args: &ArgMatches) {
    
    let read_filename  = args.get_one::<String>("read") ;
    let write_filename = args.get_one::<String>("write") ;

    let (path, opcode) = match (read_filename,write_filename) {
        (Some(f), Option::None) => (f.clone(), Opcode::Read),
        (Option::None, Some(f)) => (f.clone(), Opcode::Write),
        _ => panic!("Only --read or --write allowed"),
    };

    let filename = Path::new(&OsString::from(path.clone())).file_name().expect("no filename avail").to_str().unwrap().to_string();

    let mut buf = Vec::new();

    let mut socket = UdpSocket::bind("127.0.0.1:0").expect("Bind to interface failed");
    socket.connect("127.0.0.1:69").expect("Connection failed");

    //send request
    socket.send(PacketBuilder::new(&mut buf)
        .opcode(opcode)
        .str(&filename)
        .separator()
        .transfer_mode(TransferMode::Octet)
        .separator()
        .as_bytes()).expect("ERR  : send tftp request failed");

    let mut timeout = Timeout::new(RECV_TIMEOUT);

    loop {
        if timeout.is_timeout() {
            break;
        }

        match opcode {
            Opcode::Read => {
                let path: OsString = args.get_one::<String>("read").unwrap().into();
                let mut file = File::create(path).expect("Cannot write file");
                read_action(&mut socket, &mut file);
                break;
            }
            Opcode::Write => {
                let path: OsString = args.get_one::<String>("write").unwrap().into();
                let mut file = File::open(path).expect("Cannot write file");
                write_action(&mut socket, &mut file);
                break;
            }
            _ => panic!("not yet implemented"),
        }
    }


}

fn read_action(socket: &mut UdpSocket, file: &mut File) {
    let mut timeout    =  Timeout::new(RECV_TIMEOUT);
    let mut expected_block = 1u16;
    let mut buf: Vec<u8>        = Vec::new();
    let mut is_end        = false;

    while !is_end {
        buf.resize(protcol::MAX_PACKET_SIZE, 0);

        if timeout.is_timeout() {
            println!("timeout");
            break;
        }

        buf.resize(PACKET_SIZE_MAX, 0);
        let _                       = socket.set_read_timeout(Some(Duration::from_secs(1))); 
        let (amt, _src) = match socket.recv_from(&mut buf) {
            Ok((size,socket)) => (size,socket),
            Err(_) => {
               continue;
            }
        };

        buf.resize(amt, 0);

        if !check_datablock(&buf, expected_block) {
            continue;
        }

        //handle  data
        {
            buf.resize(amt, 0);
            let recv_data = &buf[DATA_OFFSET..];

            let _ = file.write(recv_data).expect("cannot write file");

            if recv_data.len() < DEFAULT_BLOCKSIZE {
                is_end = true;
            }
        }

        //send ACK
        let _ = socket.send(PacketBuilder::new(&mut buf)
            .opcode(Opcode::Ack)
            .number16(expected_block).as_bytes());

        expected_block = expected_block.overflowing_add(1).0;
        timeout.reset();
       
    }
}

fn write_action(socket: &mut UdpSocket, file: &mut File) {
    let mut timeout    =  Timeout::new(RECV_TIMEOUT);
    let mut expected_ack = 0u16;
    let mut buf: Vec<u8>        = Vec::new();
    let mut filebuf: Vec<u8>    = Vec::new();
    let mut is_last       = false;

    filebuf.resize(protcol::MAX_BLOCKSIZE, 0);

    while !is_last {
        if timeout.is_timeout() {
            panic!("timeout");
        }

        //check ack
        {
            buf.resize(PACKET_SIZE_MAX, 0);
            let _                       = socket.set_read_timeout(Some(Duration::from_secs(1))); 
            let (amt, _src) = match socket.recv_from(&mut buf) {
                Ok((size,socket)) => (size,socket),
                Err(_) => {
                   continue;
                }
            };

            buf.resize(amt, 0);

            let mut pp = PacketParser::new(&mut buf);

            match pp.opcode() {
                Some(Opcode::Ack) => {
                    if !pp.number16_expected(expected_ack) {
                        continue;
                    }

                },
                Some(Opcode::Error) => {
                    panic!("tftp error recv");
                },
                _ => {
                    continue;
                },
            }

            expected_ack = expected_ack.overflowing_add(1).0;
            timeout.reset();
        }

        buf.resize(protcol::MAX_PACKET_SIZE, 0);
        PacketBuilder::new(&mut buf).opcode(Opcode::Data).number16(expected_ack);

        let payload_len = match file.read(&mut filebuf[0..DEFAULT_BLOCKSIZE]) {
            Ok(len) => len,
            _          => panic!("cannot read from file"),
        };

        if payload_len != DEFAULT_BLOCKSIZE  {
            is_last = true;
        }

        buf.extend_from_slice(&filebuf[0..payload_len]);
        socket.send(&buf).expect("cannot send data");
    }
}