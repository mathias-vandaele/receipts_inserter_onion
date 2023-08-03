docker build -t receipts-inserter .
docker tag receipts-inserter:latest mathiasvandaele/receipts:latest
docker push mathiasvandaele/receipts:latest