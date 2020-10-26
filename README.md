CLI profiling tool for Cloudflare System Assignment 2020

To build, just type cargo build and run the following command:
./target/debug/cf-systems/cli
run with --help command for usage instructions

One of the main libraries I used is rustls, a library to set up TLS connections with servers. Since this was my first time using it, I took inspiration from some example code on how to configure and make tls connections. The repo can be found here: https://github.com/ctz/rustls


Profile value: make 15 requests
Output for my website ("https://new-grad-assign.amaca.workers.dev/"):
Total number of requests: 15
Total errors: 0
Fastest time: 0.088s
Slowest time: 0.202s
Median time: 0.103s
Mode time: 0.1s
Percentage of requests succeeded: 100%
Size of smallest response: 2724 bytes
Size of biggest response: 2738 bytes
Error codes recieved: none


Output for /links endpoint ("https://new-grad-assign.amaca.workers.dev/links"):
Total number of requests: 15
Total errors: 0
Fastest time: 0.077s
Slowest time: 0.086s
Median time: 0.08s
Mode time: 0.078s
Percentage of requests succeeded: 100%
Size of smallest response: 967 bytes
Size of biggest response: 981 bytes
Error codes recieved: none


Output for YouTube ("https://www.youtube.com/"):
Total number of requests: 15
Total errors: 0
Fastest time: 0.384s
Slowest time: 0.601s
Median time: 0.428s
Mode time: 0.384s
Percentage of requests succeeded: 100%
Size of smallest response: 424313 bytes
Size of biggest response: 558283 bytes
Error codes recieved: none


Output for Spotify ("https://www.spotify.com/"):
Total number of requests: 15
Total errors: 15
Fastest time: 0.141s
Slowest time: 0.152s
Median time: 0.146s
Mode time: 0.144s
Percentage of requests succeeded: 0%
Size of smallest response: 1517 bytes
Size of biggest response: 1517 bytes
Error codes recieved:
302
All of these were redirects!

Output for Google ("https://www.google.com"):
Total number of requests: 15
Total errors: 0
Fastest time: 0.24s
Slowest time: 0.425s
Median time: 0.245s
Mode time: 0.245s
Percentage of requests succeeded: 100%
Size of smallest response: 49599 bytes
Size of biggest response: 49701 bytes
Error codes recieved: none


The http response time for my website and endpoints set up using the CloudFlare network were very fast, although I suspect that some of this speed could be due to the size of the bytes recieved (which were relatively small). 
