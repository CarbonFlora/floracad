#[cfg(test)]
mod data_tests {
    use anyhow::Result;
    use floracad::datatypes::Angle;

    #[test]
    fn from_angle() {
        let angles = vec![
            "10d32\'60.1\"",
            "1d0\'0\"",
            "10d",
            "10\'",
            "10\"",
            "10\'12\"",
        ];

        for angle in angles {
            match Angle::from(angle) {
                Ok(w) => println!("O: {:?}", w),
                Err(e) => println!("Failed: {} for {}", angle, e),
            }
        }
    }

    #[test]
    fn dd_dms_dd_eq() -> Result<()> {
        let angles = vec![
            "10d32\'60.1\"",
            "1d0\'0\"",
            "10d",
            "10\'",
            "10\"",
            "10\'12\"",
        ];

        for angle in angles {
            let w1 = Angle::from(angle)?;
            let w2 = w1.to_dms();
            println!("{:?} ?= {:?}", angle, w2);
        }
        Ok(())
    }
}
