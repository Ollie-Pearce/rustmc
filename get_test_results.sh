cd test_results/

search_string="Verification complete. No errors were detected."

file_count=$(ls | wc -l)

success_count=$(grep -rl "$search_string" . | wc -l)

echo "Verification success: $success_count / $file_count"