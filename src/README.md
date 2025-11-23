## description of working of network sniffer 
- opens the nic in promiscous mode which can detect the incoming packets in the network channel
- analyses the protocols , headers and payload for any sensitive information 
- data packets are encrypted so they cannot be read until you control the server or the client

# what can a network sniffer do legitimately

1. Packet Capture

Capture raw packets from an interface

Save them as .pcap files

Replay for analysis

2. Protocol Analysis

Wireshark can decode hundreds of protocols:

TCP, UDP

HTTP/2, HTTP/3

DNS

TLS handshake

DHCP

SIP, RTP (VoIP)

3. Traffic Monitoring

Bandwidth usage

Connections by IP

Ports utilized

Latency/jitter

Packet loss

4. Security Monitoring

Sniffers can detect:

Suspicious traffic

Port scans

Anomalies

Malware communication patterns

DNS tunneling indicators

5. Debugging & Development

Developers use sniffers to:

Debug API calls

Inspect TCP retransmissions

Analyze handshake issues

Solve WebSocket connection problems

Capture failed network requests

6. Network Performance Analysis

RTT

Throughput

Congestion window analysis

Packet timing

7. VoIP & Media analysis

RTP streams

Call quality

Packet jitter

8. Wireless monitoring

Tools like Aircrack-ng can:

Capture WiFi frames

Monitor channels

Analyze RSSI and beacon frames
(but not decrypt without the WiFi password)


analyse
-- set a time for the sniffer to act and then do the work during that duration only 
-- get the interval from cmd line or options 
-- store the data in a .pcap file
-- then analyse the packets and retrieve the headers and protocols from the  