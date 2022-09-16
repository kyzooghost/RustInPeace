#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.22 - Fully Indexed CSV

// Implement an ST client FullLookupCSV that builds an array of ST objects (one for each field), with a test client that allows the user to specify the key and value fields in each query.


// Key field, key field value, value field => corresponding value field

// Array of STs
// key and value?
// ST of STs
// 'key field' => 'key value' => 'value field' => 'value value'
// 'key' = key field
// 'key2' = key value
// 'key3' = value field
// O(M * N * M) = O(M^2N) ? Seems really inefficient

// Rene solution
// Create Array of Hashmaps, for each field create a hashmap (field => string List)
// Iterate through each row, add ()



public class Exercise22_FullyIndexedCSV {

    private class FullLookupCSV {

        // Single array of Hashmaps (field => List<String>)
        private Map<String, List<String>>[] hashMapArray;

        public void buildHashMapArray(String csvFilePath) {
            In in = new In(csvFilePath);

            boolean isFirstLine = true;

            // Iterate through each row
            while (in.hasNextLine()) {
                String line = in.readLine();

                // Array of each row element
                String[] tokens = line.split(",");

                // For first line
                if (isFirstLine) {
                    // Create hashMapArray, length = # of fields, 1 field => 1 hashmap
                    hashMapArray = (HashMap<String, List<String>>[]) new HashMap[tokens.length];

                    for (int i = 0; i < hashMapArray.length; i++) {
                        // Create Hashmap for each field
                        hashMapArray[i] = new HashMap<>();
                    }

                    isFirstLine = false;
                }

                // Iterate through each field/column
                for (int keyField = 0; keyField < tokens.length; keyField++) {
                    List<String> values = new ArrayList<>();

                    // Iterate through each field/column - O(M^2) code
                    for (int valueField = 0; valueField < tokens.length; valueField++) {
                        if (valueField != keyField) {
                            values.add(tokens[valueField]);
                        }
                    }

                    // (Single row element => Array of other row elements)
                    // Key field => Key field value => value field
                    hashMapArray[keyField].put(tokens[keyField], values);
                }
            }

            // Lol, this is O(M^2 * N) code anyway
        }

        public String get(int keyField, int valueField, String query) {

            if (keyField < 0 || valueField < 0) {
                throw new IllegalArgumentException("Fields must be equal or higher than 0");
            }

            if (keyField == valueField) {
                return query;
            } else if (keyField < valueField) {
                valueField--;
            }

            // This is 3x nested symbol table anyway, but you're using an array for the first layer instead of a symbol table
            return hashMapArray[keyField].get(query).get(valueField);
        }
    }
}

fn main() {
}