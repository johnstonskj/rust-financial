echo $CRATES
echo $CARGO_TOKEN
for CRATE in ${CRATES//,/ }
do
    cargo publish --verbose --token $CARGO_TOKEN --manifest-path $CRATE/Cargo.toml
done
