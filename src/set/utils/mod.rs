pub(crate) enum IteratorState {
    Start,
    Running,
    End,
}

#[inline(always)]
pub(crate) fn advance_array_iterator(
    state: &mut IteratorState,
    array: &mut [usize],
    sizes: &[usize],
) 
{
    match state {
        IteratorState::Start => {
            *state = IteratorState::Running;
        }
        IteratorState::Running => {
            let mut i = 0;
            while i < sizes.len() {
                array[i] += 1;
                if array[i] == sizes[i] {
                    array[i] = 0;
                    i += 1;
                } else {
                    break;
                }
            }
            if i == sizes.len() {
                *state = IteratorState::End;
            }
        }
        IteratorState::End => {}
    }
}
