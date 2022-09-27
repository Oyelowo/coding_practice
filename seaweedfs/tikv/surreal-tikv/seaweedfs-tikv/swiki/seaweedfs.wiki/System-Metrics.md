SeaweedFS uses [Prometheus](https://prometheus.io/) to store the metrics and [Grafana](https://grafana.com/grafana) to visualize them. 

SeaweedFS supports both push and pull metrics.

# Push Metrics
SeaweedFS can publish metrics to [Prometheus Push Gateway](https://github.com/prometheus/pushgateway), and the gateway pass along to the Prometheus server.

Master | Volume Server | Filer => Prometheus Push Gateway => Prometheus Server => Grafana

Note: Setting Prometheus, push gateway, and Grafana can be simplified with this https://github.com/evnsio/prom-stack

## Configuration

Just add a metrics address to `weed master` or `weed server` command line options. If you have multiple masters, please add it to all the master command line options.

```
weed master -metrics.address=<protocol><prometheus_gateway_host_name>:<prometheus_gateway_port>
# example
weed master -metrics.address=localhost:9091               # Defaults to http://localhost:9091
weed master -metrics.address=https://example.com

weed server -metrics.address=<protocol><prometheus_gateway_host_name>:<prometheus_gateway_port>
# example
weed server -metrics.address=localhost:9091               # Defaults to http://localhost:9091
weed server -metrics.address=https://example.com
```

The SeaweedFS filer or volume servers will read this metrics configuration from the master, and report the metrics directly to the Prometheus Gateway. Filer and volume servers need to be restarted for the changes to take effect.

# Pull Metrics
SeaweedFS can also start with ports accepting Prometheus metrics queries.

```
weed server -metricsPort=1234
weed master -metricsPort=1234
weed volume -metricsPort=1234
weed filer  -metricsPort=1234
```

And then you can configure your Prometheus to crawl them periodically.

# Dashboard

The dashboard is shared at https://github.com/seaweedfs/seaweedfs/blob/master/other/metrics/grafana_seaweedfs.json

If you modify the dashboard, please share your revisions.

## Example Dashboard

![](https://pbs.twimg.com/media/ET7tiEcUMAIet9M?format=jpg&name=large)
