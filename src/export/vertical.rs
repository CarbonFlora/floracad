use anyhow::Result;
use chrono::Local;
use genpdf::elements::Paragraph;
use genpdf::Document;
use std::fs::File;
use std::io::prelude::*;

use crate::datatypes::*;
use crate::vertical::{calculate::ObstacleReturn, VerticalData};

impl VerticalData {
    pub fn export_txt(&self) -> Result<()> {
        let mut file = File::create(self.input_directory.clone() + ".txt")?;

        write!(file, "{}", self.to_txt()?)?;
        Ok(())
    }

    pub fn export_pdf(&self) -> Result<()> {
        let doc = self.to_pdf()?;

        doc.render_to_file(self.input_directory.clone() + ".pdf")?;
        Ok(())
    }

    fn to_txt(&self) -> Result<String> {
        let mut buf = String::new();
        let curve = self.to_vertical_curve()?;
        buf += format!("Vertical Curve\n\nCurve Details\n--\n{}", curve.dimensions).as_str();
        buf += format!("\nMajor Stations\n--\n{}", curve.stations).as_str();

        if !self.input_design_speed.is_empty() {
            match curve.is_compliant(
                self.input_design_standard,
                self.input_sight_type,
                calc_adjustment(self.sustained_downgrade),
            ) {
                Err(e) => {}
                Ok(j) => {
                    if j.0 {
                        buf += format!(
                            "\nSight Distance Validation ({:?} - {:?})\n--\n[COMPLIANT] {:.2} > {:.2}",
                            self.input_design_standard,
                            self.input_sight_type,
                            curve.dimensions.curve_length,
                            j.1
                        )
                        .as_str();
                    } else {
                        buf += format!(
                            "\nSight Distance Validation ({:?} - {:?})\n--\n[NONCOMPLIANT] {:.2} < {:.2}",
                            self.input_design_standard,
                            self.input_sight_type,
                            curve.dimensions.curve_length,
                            j.1
                        )
                        .as_str();
                    }
                }
            }
        }

        if self.sustained_downgrade {
            buf += "\nThis vertical curve experiences a sustained downgrade.";
        }

        if !self.obstacles.interval.is_empty() {
            let obstacle_calc: Vec<ObstacleReturn> = curve.obstacle_compliant(&self.obstacles);
            buf += "\n\nObstacles\n--";
            for obstacle_return in obstacle_calc {
                match obstacle_return {
                    Err(e) => {}
                    Ok(w) => {
                        if w.0 {
                            buf += format!(
                                "\n[COMPLIANT] Obstacle: {} {:?}\nActual: {}\nDelta: {:2}",
                                w.1 .0, w.1 .1, w.2, w.3
                            )
                            .as_str();
                        } else {
                            buf += format!(
                                "\n[NONCOMPLIANT] Obstacle: {} {:?}\nActual: {}\nDelta: {:2}",
                                w.1 .0, w.1 .1, w.2, w.3
                            )
                            .as_str();
                        }
                    }
                };
            }
        }

        if !self.input_station_interval.is_empty()
            && coerce_station_value(&self.input_station_interval).is_ok()
        {
            let t = coerce_station_value(&self.input_station_interval).unwrap_or_default();
            buf += format!("\n\nInterval Stations\n--\n{}", curve.interval_stations(t)).as_str();
        }

        buf += format!("\nDate of Production: {}", Local::now()).as_str();
        buf += format!("\n\nSource Data\n--\n{:?}", self).as_str();

        Ok(buf)
    }

    fn to_pdf(&self) -> Result<Document> {
        let binding = self.to_txt()?;
        let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)?;
        let mut doc = genpdf::Document::new(font_family);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);

        doc.set_title("Vertical Curve");
        doc.set_page_decorator(decorator);
        doc.set_minimal_conformance();
        doc.set_line_spacing(1.25);

        let split_text = binding.split('\n').collect::<Vec<&str>>();
        for text in split_text {
            doc.push(Paragraph::new(text));
        }

        Ok(doc)
    }
}