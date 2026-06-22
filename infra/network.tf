# Network resources — currently empty because auto-platform-api has no database
# and therefore needs no VPC peering or VPC Access connector.
#
# When a private Cloud SQL instance or other VPC-hosted resource is added,
# uncomment and extend the blocks below.

# resource "google_compute_global_address" "private_ip_range" { ... }
# resource "google_service_networking_connection" "private_vpc_connection" { ... }
# resource "google_vpc_access_connector" "connector" { ... }
