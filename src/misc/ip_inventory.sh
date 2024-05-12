echo "start address", "end address", "cidr block" > ip_inventory.csv
cat ip_inventory.txt | while read ip
do
curl -s -L \
  -H "accept: application/rdap+json" \
  https://rdap-bootstrap.arin.net/bootstrap/ip/$ip | 
  jq -r \
  '. | [.startAddress, .endAddress, "\(.cidr0_cidrs[0].v4prefix)/\(.cidr0_cidrs[0].length)"] | @csv' \
  >> ip_inventory.csv
done
