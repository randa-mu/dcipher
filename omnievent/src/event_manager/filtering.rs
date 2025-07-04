use crate::proto_types::EventOccurrenceFilter;
use crate::types::EventOccurrence;

#[derive(thiserror::Error, Debug)]
pub enum FilterError {
    #[error("invalid data index: {0}")]
    DataIndex(&'static str),

    #[error("failed to apply filter to data")]
    Apply,
}

pub fn filter_occurrences<I>(
    occurrences: I,
    filter: EventOccurrenceFilter,
) -> Result<Vec<EventOccurrence>, FilterError>
where
    I: IntoIterator<Item = EventOccurrence>,
    I::IntoIter: ExactSizeIterator,
{
    let EventOccurrenceFilter {
        block_filter,
        data_filters,
    } = filter;

    let occurrences = occurrences.into_iter();

    let _entered = tracing::info_span!("filter_occurrences", n_unfiltered = occurrences.len());

    #[cfg(feature = "timings")]
    let start = std::time::Instant::now();

    let occurrences = occurrences
        .map(|occurrence| {
            // Apply block filters
            if let Some(block_filter) = &block_filter {
                if let Some(to_block) = block_filter.to_block {
                    if occurrence.block_info.block_number >= to_block {
                        return Ok(None);
                    }
                }

                if let Some(from_block) = block_filter.from_block {
                    if occurrence.block_info.block_number < from_block {
                        return Ok(None);
                    }
                }
            }

            // Apply event data filters
            for data_filter in &data_filters {
                let data_index = usize::try_from(data_filter.data_index)
                    .map_err(|_| FilterError::DataIndex("failed to convert index"))?;

                // Make sure that the filter references a valid field
                let Some(data_field) = occurrence.data.get(data_index) else {
                    Err(FilterError::DataIndex("invalid data index for occurrence"))?
                };

                let Some(filter) = &data_filter.filter else {
                    // No filters => continue applying filters
                    continue;
                };

                match filter.apply(&data_field.data) {
                    Some(true) => {
                        // Continue applying filters
                    }

                    Some(false) => {
                        // Failed to pass filter, return None for this occurrence
                        return Ok(None);
                    }

                    None => {
                        // Could not apply filter, abort
                        return Err(FilterError::Apply);
                    }
                }
            }

            Ok(Some(occurrence))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    #[cfg(not(feature = "timings"))]
    tracing::debug!(
        n_filtered = occurrences.len(),
        "Finished filtering occurrences"
    );
    #[cfg(feature = "timings")]
    {
        let elapsed = start.elapsed();
        tracing::debug!(
            n_filtered = occurrences.len(),
            duration_secs = elapsed.as_secs_f64(),
            "Finished filtering occurrences"
        );
    }

    Ok(occurrences)
}
