use plotpy::{Curve, Plot, StrError};
use crate::{statistics_util::data_to_vector, GraphInfo};

use crate::file_util::get_save_path;

pub fn make_2d_curve_from_data(x_data: &Vec<f64>, y_data: &Vec<f64>) -> Result<Curve, StrError> {
        // configure curve
        let mut curve = Curve::new();
        curve.set_line_width(2.0);
    
        // add points
        curve.points_begin();
        for i in 0..std::cmp::min(x_data.len(), y_data.len()) {
            curve.points_add(x_data[i], y_data[i]);
        }
        curve.points_end();

        return Ok(curve);
}

pub fn make_3d_curve_from_data(x_data: &Vec<f64>, y_data: &Vec<f64>, z_data: &Vec<f64>) -> Result<Curve, StrError> {
    let mut curve = Curve::new();
    curve.set_line_width(2.0);
    
    // add points
    curve.points_3d_begin();
    for i in 0..std::cmp::min(x_data.len(), y_data.len()) {
        curve.points_3d_add(x_data[i], y_data[i], z_data[i]);
    }
    curve.points_3d_end();
    return Ok(curve);
}

pub fn make_graph(graph_info: GraphInfo) -> Result<(), StrError> {

    // add curve to plot
    let mut plot = Plot::new();

    let x_data = data_to_vector(graph_info.x_data.as_str());
    let y_data = data_to_vector(graph_info.y_data.as_str());
    if graph_info.is_2d {
        // configure curve
        let curve = make_2d_curve_from_data(&x_data, &y_data)?;
        plot
            .set_title(graph_info.title.as_str())
            .grid_and_labels(&graph_info.x_axis_label, &graph_info.y_axis_label);
        plot.add(&curve);
    }
    else {
        let z_data = data_to_vector(graph_info.y_data.as_str());
        let curve = make_3d_curve_from_data(&x_data, &y_data, &z_data)?;
        plot.set_subplot_3d(1, 1, 1)
            .set_label_x(&graph_info.x_axis_label)
            .set_label_y(&graph_info.y_axis_label)
            .set_label_z(&graph_info.z_axis_label)
            .set_title(&graph_info.title)
            .add(&curve);
    }

    // save figure
    match get_save_path() {
        Some(mut path) => {
            path.push(format!("{}_graph", graph_info.title));
            plot.save_and_show(path.as_path())?;
            return Ok(());
        },
        None => {
            return Err("Invalid Path");
        },
    }
}

#[cfg(test)]
mod graph_maker_tests {
    use super::*;

    #[test] 
    fn make_simple_2d_curve() {
        let vec: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

        assert!(
            make_2d_curve_from_data(&vec, &vec).is_ok()
        );
    }

    #[test] 
    fn make_simple_3d_curve() {
        let vec: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

        assert!(
            make_3d_curve_from_data(&vec, &vec, &vec).is_ok()
        );
    }
}