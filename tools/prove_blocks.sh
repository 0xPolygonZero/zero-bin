#!/bin/bash

# Args:
# 1 --> Start block idx
# 2 --> End block index (inclusive)
# 3 --> Rpc endpoint:port (eg. http://35.246.1.96:8545)

export RUST_BACKTRACE=1
export RUST_LOG=plonky2=trace,plonky2_evm=trace,protocol_decoder=trace

export ARTITHMETIC_CIRCUIT_SIZE="16..28"
export BYTE_PACKING_CIRCUIT_SIZE="9..28"
export CPU_CIRCUIT_SIZE="12..28"
export KECCAK_CIRCUIT_SIZE="14..28"
export KECCAK_SPONGE_CIRCUIT_SIZE="9..28"
export LOGIC_CIRCUIT_SIZE="12..28"
export MEMORY_CIRCUIT_SIZE="17..30"

PROOF_OUTPUT_DIR="proofs"
ALWAYS_WRITE_LOGS=0 # Change this to `1` if you always want logs to be written.

TOT_BLOCKS=$(($2-$1+1))

echo "Proving blocks ${1}..=${2}... (Total: ${TOT_BLOCKS})"
mkdir -p proofs/

for ((i=$1; i<=$2; i++))
do
    echo "Proving block ${i}..."

    out_idx=$(printf "%05d" $i)

    OUT_PROOF_PATH="${PROOF_OUTPUT_DIR}/b${out_idx}.zkproof"
    OUT_LOG_PATH="${PROOF_OUTPUT_DIR}/b${out_idx}.log.bz2"

    if [[ $i -gt 1 ]]; then
        prev_proof_num=$((i-1))
	prev_proof_num_idx=$(printf "%05d" $prev_proof_num)
        PREV_PROOF_EXTRA_ARG="-f ${PROOF_OUTPUT_DIR}/b${prev_proof_num_idx}.zkproof"
    fi

    # echo "cheese"
    # echo "cargo r --release --bin leader -- --runtime in-memory jerigon --rpc-url "$3" --block-number $i --proof-output-path $OUT_PROOF_PATH $PREV_PROOF_EXTRA_ARG > $OUT_LOG_PATH 2>&1"
    cargo r --release --bin leader -- \
	  --runtime in-memory jerigon \
	  --rpc-url "$3" \
	  --block-number $i \
	  --proof-output-path $OUT_PROOF_PATH \
	  $PREV_PROOF_EXTRA_ARG 2>&1 | bzip2 > $OUT_LOG_PATH
    # touch $OUT_LOG_PATH
    # touch $OUT_PROOF_PATH

    if [[ -e $OUT_PROOF_PATH ]]; then
        if [[ $ALWAYS_WRITE_LOGS -ne 0 ]]; then
            rm $OUT_LOG_PATH
        fi
    else
	echo "Block ${i} errored. See ${OUT_LOG_PATH} for more details."
	exit 1
    fi
done

echo "Successfully generated ${TOT_BLOCKS} proofs!"
