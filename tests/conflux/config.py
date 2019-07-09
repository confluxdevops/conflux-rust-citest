from . import trie
from eth_utils import decode_hex

default_config = dict(
    GENESIS_DIFFICULTY=0,
    GENESIS_PREVHASH=b'\x00' * 32,
    GENESIS_COINBASE=b'\x00' * 20,
    GENESIS_PRI_KEY=decode_hex("46b9e861b63d3509c88b7817275a30d22d62c8cd8fa6486ddee35ef0d8e0495f"),
    TOTAL_COIN= 5 * 10**9 * 10**18 * 10**6,
    GENESIS_STATE_ROOT=trie.state_root(delta_root=decode_hex("0x5a79b0adf83ff7ea83c7862e825e1ad64af0c4492a4e0f7a48610cb130fbfb14")),
    GENESIS_RECEIPTS_ROOT=decode_hex("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
    GENESIS_AUTHOR=decode_hex("0x0000000000000000000000000000000000000008"),
)