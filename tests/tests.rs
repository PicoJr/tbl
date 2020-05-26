extern crate tbl;

#[cfg(test)]
mod tests {
    use tbl::{Bound, Renderer, TBLError};

    #[test]
    fn test_empty() {
        let data: Vec<Bound> = vec![];
        let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
            .with_length(8)
            .render();
        assert_eq!(rendered.unwrap(), "        ");
    }

    #[test]
    fn test_intersection() {
        let data: Vec<Bound> = vec![(0., 2.), (1., 4.)];
        let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
            .with_length(8)
            .render();
        assert!(match rendered {
            Err(TBLError::Intersection(_, _)) => true,
            _ => false,
        })
    }

    #[test]
    fn test_ok() {
        let data: Vec<Bound> = vec![(0., 2.), (3., 4.)];
        let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
            .with_length(8)
            .render();
        assert_eq!(rendered.unwrap(), "====  ==")
    }

    #[test]
    fn test_length_0() {
        let data: Vec<Bound> = vec![(0., 2.), (3., 4.)];
        let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
            .with_length(0)
            .render();
        assert_eq!(rendered.unwrap(), "")
    }

    #[test]
    fn test_boundaries_ok() {
        let data: Vec<Bound> = vec![(1., 2.), (3., 4.)];
        let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
            .with_length(10)
            .with_boundaries((0., 5.))
            .render();
        assert_eq!(rendered.unwrap(), "  ==  ==  ")
    }

    #[test]
    fn test_boundaries_ok2() {
        let data: Vec<Bound> = vec![(1., 2.), (3., 4.)];
        for length in 0..100 {
            let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
                .with_length(length)
                .with_boundaries((0., 3.))
                .render();
            assert_eq!(rendered.unwrap().len(), length);
        }
    }
}
