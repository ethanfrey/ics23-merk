#[cfg(test)]
mod tests {
    use merk::{Merk, Result, TreeBatchEntry, TreeOp};

    #[test]
    fn loading() -> Result<()> {
        let mut db = Merk::open("./merk.db")?;

        let mut batch: Vec<TreeBatchEntry> = vec![
            (b"key", TreeOp::Put(b"value")),
            (b"key2", TreeOp::Put(b"value2")),
            (b"key3", TreeOp::Put(b"value3")),
        ];
        db.apply(&mut batch)?;

        if let Some(tree) = &db.tree {
            let hash = tree.node().hash();
            assert_eq!("b18fa953ef750f27c04b1119d96b9932a7c8806d", hex::encode(&hash));
            let v = tree.get(b"key");
            assert_eq!(v, b"value2");
        } else {
            assert!(false, "foo");
        }

        let v = db.get(b"key");
        assert_eq!(v, b"value2");


//        let mut batch: Vec<TreeBatchEntry> = vec![
//            (b"key2", TreeOp::Delete),
//        ];
//        db.apply(&mut batch)?;

//        println!("height: {}", &db.height());

        // Do cleanup
        db.destroy()
    }
}
