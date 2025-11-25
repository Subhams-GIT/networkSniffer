# Network Sniffer -- Feature Specification

cmd args
- filter by protocol
- filter by ports
- filter by ip address

## 1. Core Features (Beginner)

### **Packet Capture**

-   Capture packets from selected network interface\
-   Save packets to `.pcap`\
-   Import packets from existing `.pcap` files -> option

### **Packet Filtering**

-   Filter by protocol (TCP, UDP, ICMP)\
-   Filter by ports\
-   Filter by IP address\
-   Basic BPF-like syntax

### **Basic Packet Parsing**

-   Ethernet header\
-   IPv4/IPv6 header\
-   TCP/UDP header\
-   ICMP types\
-   Raw payload view

------------------------------------------------------------------------

## 2. Intermediate Features

### **Protocol Decoding**

-   HTTP (method, URL, status)\
-   DNS (queries & responses)\
-   TLS handshake metadata\
-   ARP parsing

### **Live Traffic Dashboard**

-   Packet count\
-   Active connections\
-   Top talkers\
-   Protocol distribution

### **Connection Tracking**

-   TCP handshake tracking\
-   Reset detection\
-   Retransmission analysis

### **Logging**

-   Export metadata to JSON or CSV\
-   Persistent logs with timestamps

------------------------------------------------------------------------

## 3. Advanced Features

### **Flow Analysis**

-   Group packets into flows\
-   Measure RTT\
-   Detect slow connections\
-   Bandwidth estimation

### **Basic Intrusion Detection**

-   Port scan detection\
-   Malformed packet alerts\
-   High‑traffic anomaly detection

### **Performance Metrics**

-   Jitter\
-   Packet loss\
-   Throughput charts

### **TCP Stream Reassembly**

-   Rebuild TCP streams\
-   Reconstruct HTTP streams (headers only)

------------------------------------------------------------------------

## 4. Expert Features

### **TLS Metadata Analyzer**

-   TLS version\
-   Cipher suites\
-   SNI\
-   Certificate metadata

### **Wireless Features (Monitor Mode)**

-   Capture WiFi management frames\
-   Channel scanning\
-   RSSI analysis

### **Visualization & UI**

-   Real‑time charts\
-   IP heatmaps\
-   Protocol distribution graphs

------------------------------------------------------------------------

## 5. Optional Enhancements

### **Machine Learning Traffic Classification**

-   Traffic type classification\
-   Anomaly detection via clustering

### **Plugin Architecture**

-   Custom parsers\
-   Extendable modules

### **Multi‑Interface Sniffing**

-   Capture from multiple NICs simultaneously
