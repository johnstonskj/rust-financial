for CRATE in ${CRATES//,/ }
do
    cargo doc --verbose --package $CRATE --no-deps
done
