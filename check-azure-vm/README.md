# Check Azure VM Status

## Json Sample

```bash
cat azure-vm-status.json
```

## Output

```bash
# pichuang @ rush in ~/ms-workspace/azure-rust-lab/check-azure-vm on git:master x [23:13:06]
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s

# pichuang @ rush in ~/ms-workspace/azure-rust-lab/check-azure-vm on git:master x [23:13:10]
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/check-azure-vm`
.name: "vm-core-taiwannorth"
.properties.provisioningState: "Succeeded"
.properties.timeCreated: "2024-03-28T13:05:35.7367577+00:00"
```