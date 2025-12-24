iwasi=target/wasm32-wasip1/release-wasi/rawzip2entry1st.wasm

wasm-opt \
	-Oz \
	-o rawzip2entry1st-opt.wasm \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	"${iwasi}"
