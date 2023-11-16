# Open telemetry in Rust

Run application and telemetry tools:

    docker run -d -p9091:9090 -v ./prometheus.yaml:/etc/prometheus/prometheus.yaml prom/prometheus
    docker run -d -p3000:3000 -e GF_SECURITY_ADMIN_USER=admin -e GF_SECURITY_ADMIN_PASSWORD=admin grafana/grafana:latest
    docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
    cargo run -p sub
    cargo run -p add

Send http requests:

    http POST http://localhost:8080/add --raw '[1,2]' 
    http POST http://localhost:8081/sub --raw '[1,2]'

## Todo:
 - add instrumentation to code
 - add metrics
