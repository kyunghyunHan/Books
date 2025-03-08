use ndarray::{Array, Array1};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
fn ch2_0(){
    let n = 1000000;
    let m = 2;  // 3에서 2로 변경

    let mut heads: Array1<f64> = Array::zeros(m + 1);
    for i in 0..n {
        let random_array: Array1<usize> = Array::random(m, Uniform::new(0, 2));
        let counts = bincount(&random_array, 2);
        let h = counts[0];
        heads[m - h] += 1.;
    }
    let prob = &heads / n as f64;
    println!("Probabilities: {:.6}", prob);
}
fn ch2_1() {
    let n = 1000000;
    let m = 3;

    let mut heads: Array1<f64> = Array::zeros(m + 1);
    for i in 0..n {
        let random_array: Array1<usize> = Array::random(m, Uniform::new(0, 2));
        let counts = bincount(&random_array, 2);
        let h = counts[0];
        heads[m - h] += 1.;
    }
    let prob = &heads / n as f64;
    println!("Probabilities: {:.6}", prob);
}

fn ch2_2() {
    for n in 0..300 {
        if (364_f64 / 365_f64).powi(n) < 0.5 {
            println!("{}", n);
            break;
        }
    }
}
fn ch2_3() {
    let mut count = 0;
    for i in 0..100000 {
        let a: Array1<usize> = Array::random(1, Uniform::new(0, 364));
        let b: Array1<usize> = Array::random(1, Uniform::new(0, 364));
        if a == b {
            count += 1;
        }
    }
    let probability = count as f64 / 100000 as f64;
    println!("Probability of match: {:.6}", probability);
    // around 0.3 %
}

fn ch2_4() {
    for m in 2..31 {
        let mut counts = 0;
        for n in 0..10000 {
            let mut count = 0;
            let b: Array1<usize> = Array::random(m, Uniform::new(0, 364));
            for i in 0..m {
                for j in 0..m {
                    if i != j && b[i] == b[j] {
                        count += 1;
                    }
                }
            }
            if count != 0 {
                counts += 1
            }
        }
        println!("{:2} {:.6}", m, counts as f64 / 100000 as f64)
    }
}
fn ch2_5() -> Result<(), Box<dyn std::error::Error>> {
    use ndarray::Array1;
    use ndarray_rand::rand::rngs::StdRng;
    use ndarray_rand::rand::SeedableRng;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use plotters::prelude::*;

    const OUT_FILE_NAME: &str = "./assets/img/birthday_problem.png";

    let mut probabilities = Vec::new();
    let max_people = 30; // 1부터 50명까지 계산

    // 각 m명에 대한 생일 충돌 확률 계산
    for m in 1..=max_people {
        let mut matches = 0;
        let trials = 10000; // 10,000번의 시뮬레이션

        for _ in 0..trials {
            // m명의 랜덤 생일 생성 (0-363)
            let birthdays: Array1<usize> = Array::random(m, Uniform::new(0, 364));

            // 생일 충돌 확인
            let mut has_match = false;
            'outer: for i in 0..m {
                for j in (i + 1)..m {
                    if birthdays[i] == birthdays[j] {
                        has_match = true;
                        break 'outer;
                    }
                }
            }

            if has_match {
                matches += 1;
            }
        }

        let probability = matches as f64 / trials as f64;
        probabilities.push((m as f64, probability));
        println!("사람 수: {:2}, 확률: {:.6}", m, probability);
    }

    // 이론적인 확률 계산 (비교용)
    let theoretical: Vec<(f64, f64)> = (1..=max_people)
        .map(|m| {
            let prob = 1.0 - (0..m).fold(1.0, |acc, i| acc * (364.0 - i as f64) / 364.0);
            (m as f64, prob)
        })
        .collect();

    // 그래프 그리기
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "생일 문제: m명의 사람들 중 생일이 같은 2명이 있을 확률",
            ("sans-serif", 20),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..(max_people as f64 + 1.0), 0f64..1.05f64)?;

    chart
        .configure_mesh()
        .x_labels(15)
        .y_labels(10)
        .x_desc("사람 수 (m)")
        .y_desc("확률")
        .draw()?;

    // 시뮬레이션 결과 그리기
    chart
        .draw_series(LineSeries::new(
            probabilities.iter().map(|&(x, y)| (x, y)),
            &BLUE,
        ))?
        .label("시뮬레이션 결과")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 이론적 확률 그리기
    chart
        .draw_series(LineSeries::new(
            theoretical.iter().map(|&(x, y)| (x, y)),
            &RED.mix(0.5),
        ))?
        .label("이론적 확률")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));

    // 50% 확률 지점 표시를 위한 가로선
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.5), (max_people as f64, 0.5)],
        &BLACK.mix(0.3),
    ))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // 50% 확률이 되는 지점 찾기
    let threshold_point = theoretical.iter().find(|&&(_, prob)| prob >= 0.5).unwrap();
    // .unwrap_or(&(max_people as f64, 1.0));

    // 50% 임계점 표시
    chart.draw_series(PointSeries::of_element(
        vec![(threshold_point.0, threshold_point.1)],
        7,
        &GREEN,
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style.filled())
                + Text::new(
                    format!("약 {}명에서 50% 확률", threshold_point.0 as usize),
                    (10, -15),
                    ("sans-serif", 15).into_font(),
                )
        },
    ))?;

    root.present()?;
    println!("그래프가 {}에 저장되었습니다", OUT_FILE_NAME);

    Ok(())
}

fn ch2_6() {
    let mut nb = 0;
    let n = 100000;
    for i in 0..n {
        let s: Array1<usize> = Array::random(3, Uniform::new(0, 50));
        let mut fail = false;
        for t in 0..3 {
            if s[t] < 4 {
                fail = true;
            }
        }
        if !fail {
            nb += 1;
        }
    }
    println!("No Boston in the fall {:.4}", nb as f64 / n as f64);
}
pub fn example() {
    ch2_0();
}
fn bincount(arr: &Array1<usize>, minlength: usize) -> Array1<usize> {
    let max_val = arr.iter().max().copied().unwrap_or(0);
    let length = std::cmp::max(max_val + 1, minlength);

    let mut counts: Array1<usize> = Array::zeros(length);

    for &val in arr.iter() {
        counts[val] += 1;
    }

    counts
}
