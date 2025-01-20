pub struct State {
    pub cells: ndarray::Array<u8, ndarray::Ix2>,
}

impl<I: ca_formats::Input> From<ca_formats::rle::Rle<I>> for State {
    fn from(rle: ca_formats::rle::Rle<I>) -> Self {
        let width: usize;
        let height: usize;
        let mut x_start = 0;
        let mut y_start = 0;

        match rle.header_data() {
            Some(ca_formats::rle::HeaderData { x, y, .. }) => {
                width = *x as usize;
                height = *y as usize;
            }
            None => panic!("Could not read RLE header data."),
        }

        match rle.cxrle_data() {
            Some(ca_formats::rle::CxrleData {
                pos: Some((x, y)), ..
            }) => {
                x_start = x.clone();
                y_start = y.clone();
            }
            None => (),
            data => panic!("Could not read RLE context data: {:?}", data),
        }

        let mut cells = ndarray::Array::zeros((height, width));

        for cell in rle {
            match cell {
                Ok(ca_formats::CellData {
                    position: (x, y),
                    state,
                }) => {
                    let x: usize = (x - x_start).try_into().unwrap();
                    let y: usize = (y - y_start).try_into().unwrap();

                    cells[(y, x)] = state;
                }
                Err(error) => panic!("{}", error),
            };
        }

        State {
            cells: cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

    #[test]
    fn test_from_rle() {
        let rle = ca_formats::rle::Rle::new("x = 4, y = 3, rule = WireWorld\n.BA$C2.C$.2C!").unwrap();

        let state = State::from(rle);

        assert_eq!(state.cells.shape(), [3,4]);

        assert_eq!(state.cells[(0, 1)], 2);
        assert_eq!(state.cells[(0, 2)], 1);
        assert_eq!(state.cells[(1, 0)], 3);
    }

    #[test]
    fn test_from_rle_with_negative_indexes() {
        let rle = ca_formats::rle::Rle::new("#CXRLE Pos=-1,-1\nx = 4, y = 3, rule = WireWorld\n.BA$C2.C$.2C!").unwrap();

        let state = State::from(rle);

        assert_eq!(state.cells.shape(), [3,4]);

        assert_eq!(state.cells[(0, 1)], 2);
        assert_eq!(state.cells[(0, 2)], 1);
        assert_eq!(state.cells[(1, 0)], 3);
    }
}
