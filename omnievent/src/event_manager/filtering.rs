use crate::proto_types::EventOccurrenceFilter;
use crate::types::EventOccurrence;

#[derive(thiserror::Error, Debug)]
pub enum FilterError {
    #[error("invalid data index: {0}")]
    DataIndex(&'static str),

    #[error("failed to apply filter to data")]
    Apply,
}

#[tracing::instrument(skip_all, fields(n_unfiltered = occurrences.len()))]
pub fn filter_occurrences(
    occurrences: impl IntoIterator<Item = EventOccurrence> + ExactSizeIterator,
    filter: EventOccurrenceFilter,
) -> Result<Vec<EventOccurrence>, FilterError> {
    let EventOccurrenceFilter {
        block_filter,
        data_filters,
    } = filter;

    let occurrences = occurrences
        .into_iter()
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

    tracing::debug!(
        n_filtered = occurrences.len(),
        "Finished filtering occurrences"
    );

    Ok(occurrences)
}
