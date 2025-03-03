#!/bin/bash
# LXCコンテナのIDリストを取得

container_ids=$(pct list | awk '{if(NR>1)print $1}')  # ヘッダーを除外

echo "コンテナID, ホスト名, IPアドレス"

for id in $container_ids; do
  hostname=$(pct exec $id -- hostname)
  ip_address=$(pct config $id | grep -oP 'net\d+:.*?ip=\K[\d.]+')
  echo "$id, $hostname, $ip_address"
done