use plotters::chart::ChartBuilder;
use plotters::prelude::{BitMapBackend, Color, IntoDrawingArea, IntoFont, Palette, Rectangle, SeriesLabelPosition, WHITE};
use plotters::series::LineSeries;
use plotters::style::{BLACK, Palette99};

pub fn draw_chart<T>(data: Vec<(&str, Vec<T>)>, n_range: impl Iterator<Item=usize> + Clone, name: &str)
    where T: Clone + PartialOrd, f64: From<T> {
    let file = format!("charts/chart_{}.png", name);

    let mut n_range_copy = n_range.clone();
    let first = n_range_copy.next().unwrap() as f64;
    let last = n_range_copy.last().unwrap() as f64;
    let x_range = first..last;

    let max = data.iter().map(|x| x.1.iter().max_by(|a,b| a.partial_cmp(b).unwrap()))
        .max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().unwrap().clone().into();
    let y_range = 0.0..max;



    let mut drawing_area = BitMapBackend::new(&file, (1280, 720)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .margin(5)
        .caption(name, ("Calibri", 40).into_font())
        .set_all_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();


    for i in data.iter().enumerate() {
        let (num, (name, vals)) = i;
        ctx.draw_series(
            LineSeries::new(n_range.clone().zip(vals.iter()).map(|(x,y)| (x as f64, y.clone().into() )), Palette99::pick(num))).unwrap()
            .label(*name).legend(move |(x, y)| Rectangle::new([(x, y - 8), (x + 15, y + 7)], Palette99::pick(num).filled()));
    }

    ctx.configure_series_labels().border_style(&BLACK).label_font(("Calibri", 20)).position(SeriesLabelPosition::UpperLeft).background_style(&WHITE).draw().unwrap();
}