## Problems (and Solutions)

---

**Problem:** Block definition between gateway and RPC differ only in a few property names. So `api::gen::BlockWithTxs` needs to be reused.

**Solution:** Align property names between gateway and RPC using `#[serde(alias = "...")]` in `api::gen::*`;

---

**Problem:** Some of `FUNCTION_CALL` transactions returned from the gateway (observed only for ones with `entry_point_selector` field) do not contain `nonce` field. 

**Solution:** Workaround with using `util::patch_*` functions.

---

**Problem:** Block `state_root` needs to be a Felt, but does not have `"0x"` prefix.

**Solution:** Workaround in `api::gen::felt::Felt::try_new`.

---

<!---

**Problem:** 

**Solution:** 

---
-->
