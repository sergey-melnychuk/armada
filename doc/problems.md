## Problems (and Solutions)

---

**Problem:** Some transactions in the `pending` block (specifically `FUNCTION_CALL` ones) sometimes do not have a `nonce` field present (which is a required field). Thus seamless support of both RPC and Gateway for `BlockWithTxs` is problematic for the `pending` block, however for hased/indexed/latest block the JSON schemas differ only by a names of a few fields. (Excluding `state_root`, which in Gateway for some reason returned withoud `"0x"` prefix).

**Solution:** TODO: For now `pending` block is not supported.

---

**Problem:** ?

**Solution:** !

---
