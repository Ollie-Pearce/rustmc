cd test_results/

file_count=$(ls | wc -l)

success_search_string="Verification complete. No errors were detected."
success_count=$(grep -rl "$success_search_string" . | wc -l)

echo "Verification success: $success_count / $file_count"

uninitialised_read_string="Error: Attempt to read from uninitialized memory!"
uninitialised_read_count=$(grep -rl "$uninitialised_read_string" . | wc -l)

echo "Uninitialised read errors: $uninitialised_read_count / $file_count"

no_entry_string="ERROR: Could not find program's entry point function!"
no_entry_count=$(grep -rl "$no_entry_string" . | wc -l)

echo "No entry point errors: $no_entry_count / $file_count"

external_function_string="ERROR: Tried to execute an unknown external function:"
external_function_count=$(grep -rl "$external_function_string" . | wc -l)

echo "External function errors: $external_function_count / $file_count"

external_address_string="LLVM ERROR: Could not resolve external global address:"
external_address_count=$(grep -rl "$external_address_string" . | wc -l)

echo "External address errors: $external_address_count / $file_count"

ilist_iterator_string="llvm::ilist_iterator_w_bits"
ilist_iterator_count=$(grep -rl "$ilist_iterator_string" . | wc -l)

echo "ilist iterator errors: $ilist_iterator_count / $file_count"