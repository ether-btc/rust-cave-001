#!/usr/bin/env python3
"""Generate a fixed src/lib.rs with all the necessary changes."""

# Read the entire base file
with open('src/lib.rs', 'r') as f:
    content = f.read()

# 1. Expand verb conjugation map
old_verb_map = """let verb_conjugations: std::collections::HashMap<&str, &str> = [
        ("thrown", "threw"),
        ("eaten", "ate"),
        ("written", "wrote"),
        ("seen", "saw"),
        ("done", "did"),
        ("given", "gave"),
        ("taken", "took"),
        ("made", "made"),
        ("found", "found"),
        ("told", "told"),
        ("called", "called"),
        ("used", "used"),
        ("asked", "asked"),
        ("wanted", "wanted"),
        ("needed", "needed"),
        ("looked", "looked"),
        ("worked", "worked"),
        ("played", "played"),
        ("moved", "moved"),
        ("lived", "lived"),
        ("believed", "believed"),
        ("happened", "happened"),
        ("changed", "changed"),
        ("showed", "showed"),
        ("watched", "watched"),
        ("followed", "followed"),
        ("stopped", "stopped"),
        ("created", "made"),
        ("brought", "brought"),
        ("heard", "heard"),
        ("held", "held"),
        ("sent", "sent"),
        ("built", "built"),
        ("understood", "understood"),
        ("drawn", "drew"),
        ("grown", "grew"),
        ("flown", "flew"),
        ("broken", "broke"),
        ("sung", "sang"),
        ("drunk", "drank"),
        ("sunk", "sank"),
        ("spun", "spun"),
        ("run", "ran"),
        ("read", "read"),
        ("cut", "cut"),
        ("put", "put"),
        ("set", "set"),
        ("shut", "shut"),
        ("cost", "cost"),
        ("hurt", "hurt"),
        ("let", "let"),
        ("regretted", "regretted"),
        ("optimized", "optimized"),
        ("analyzed", "analyzed"),
        ("processed", "processed"),
        ("updated", "updated"),
        ("deleted", "deleted"),
        ("inserted", "inserted"),
        ("selected", "selected"),
        ("filtered", "filtered"),
        ("sorted", "sorted"),
        ("joined", "joined"),
    ]
    .iter()
    .cloned()
    .collect();"""

new_verb_map = """let verb_conjugations: std::collections::HashMap<&str, &str> = [
        ("thrown", "threw"),
        ("eaten", "ate"),
        ("written", "wrote"),
        ("seen", "saw"),
        ("done", "did"),
        ("given", "gave"),
        ("taken", "took"),
        ("made", "made"),
        ("found", "found"),
        ("told", "told"),
        ("called", "called"),
        ("used", "used"),
        ("asked", "asked"),
        ("wanted", "wanted"),
        ("needed", "needed"),
        ("looked", "looked"),
        ("worked", "worked"),
        ("played", "played"),
        ("moved", "moved"),
        ("lived", "lived"),
        ("believed", "believed"),
        ("happened", "happened"),
        ("changed", "changed"),
        ("showed", "showed"),
        ("watched", "watched"),
        ("followed", "followed"),
        ("stopped", "stopped"),
        ("created", "made"),
        ("brought", "brought"),
        ("heard", "heard"),
        ("held", "held"),
        ("sent", "sent"),
        ("built", "built"),
        ("understood", "understood"),
        ("drawn", "drew"),
        ("grown", "grew"),
        ("flown", "flew"),
        ("broken", "broke"),
        ("sung", "sang"),
        ("drunk", "drank"),
        ("sunk", "sank"),
        ("spun", "spun"),
        ("run", "ran"),
        ("read", "read"),
        ("cut", "cut"),
        ("put", "put"),
        ("set", "set"),
        ("shut", "shut"),
        ("cost", "cost"),
        ("hurt", "hurt"),
        ("let", "let"),
        ("regretted", "regretted"),
        ("optimized", "optimized"),
        ("analyzed", "analyzed"),
        ("processed", "processed"),
        ("updated", "updated"),
        ("deleted", "deleted"),
        ("inserted", "inserted"),
        ("selected", "selected"),
        ("filtered", "filtered"),
        ("sorted", "sorted"),
        ("joined", "joined"),
        // Common irregular verbs
        ("began", "began"),  // begin → began (note: participle is begun)
        ("blew", "blew"),
        ("broke", "broke"),
        ("chose", "chose"),
        ("came", "came"),
        ("crept", "crept"),
        ("drew", "drew"),
        ("drove", "drove"),
        ("ate", "ate"),
        ("fell", "fell"),
        ("flew", "flew"),
        ("forgot", "forgot"),
        ("froze", "froze"),
        ("grew", "grew"),
        ("knew", "knew"),
        ("leapt", "leapt"),
        ("lost", "lost"),
        ("met", "met"),
        ("paid", "paid"),
        ("rang", "rang"),
        ("rose", "rose"),
        ("sang", "sang"),
        ("sank", "sank"),
        ("spoke", "spoke"),
        ("stank", "stank"),
        ("stole", "stole"),
        ("strode", "strode"),
        ("swelled", "swelled"),
        ("swam", "swam"),
        ("took", "took"),
        ("tore", "tore"),
        ("wore", "wore"),
        ("wove", "wove"),
        ("wrote", "wrote"),
        // More irregular verbs
        ("bit", "bit"),
        ("bled", "bled"),
        ("blew", "blew"),
        ("bred", "bred"),
        ("brought", "brought"),
        ("built", "built"),
        ("bought", "bought"),
        ("caught", "caught"),
        ("chose", "chose"),
        ("cost", "cost"),
        ("cut", "cut"),
        ("dealt", "dealt"),
        ("dug", "dug"),
        ("dreamed", "dreamed"),
        ("drank", "drank"),
        ("drove", "drove"),
        ("ate", "ate"),
        ("fell", "fell"),
        ("fed", "fed"),
        ("felt", "felt"),
        ("fought", "fought"),
        ("found", "found"),
        ("fled", "fled"),
        ("forbade", "forbade"),
        ("forgot", "forgot"),
        ("forgave", "forgave"),
        ("forsook", "forsook"),
        ("froze", "froze"),
        ("got", "got"),
        ("gave", "gave"),
        ("went", "went"),
        ("ground", "ground"),
        ("grew", "grew"),
        ("hung", "hung"),
        ("had", "had"),
        ("heard", "heard"),
        ("held", "held"),
        ("hid", "hid"),
        ("hit", "hit"),
        ("held", "held"),
        ("kept", "kept"),
        ("knew", "knew"),
        ("laid", "laid"),
        ("led", "led"),
        ("left", "left"),
        ("lent", "lent"),
        ("let", "let"),
        ("lay", "lay"),
        ("lost", "lost"),
        ("made", "made"),
        ("meant", "meant"),
        ("met", "met"),
        ("paid", "paid"),
        ("pled", "pled"),
        ("put", "put"),
        ("quit", "quit"),
        ("read", "read"),
        ("rode", "rode"),
        ("rang", "rang"),
        ("rose", "rose"),
        ("ran", "ran"),
        ("saw", "saw"),
        ("said", "said"),
        ("sang", "sang"),
        ("sank", "sank"),
        ("sat", "sat"),
        ("saw", "saw"),
        ("said", "said"),
        ("sold", "sold"),
        ("sent", "sent"),
        ("set", "set"),
        ("sewed", "sewed"),
        ("shook", "shook"),
        ("shrank", "shrank"),
        ("shot", "shot"),
        ("showed", "showed"),
        ("shrank", "shrank"),
        ("sang", "sang"),
        ("sank", "sank"),
        ("slid", "slid"),
        ("split", "split"),
        ("spoke", "spoke"),
        ("spent", "spent"),
        ("spun", "spun"),
        ("sprang", "sprang"),
        ("stood", "stood"),
        ("stole", "stole"),
        ("stuck", "stuck"),
        ("stung", "stung"),
        ("stank", "stank"),
        ("strided", "strided"),
        ("struck", "struck"),
        ("sought", "sought"),
        ("swore", "swore"),
        ("swept", "swept"),
        ("swelled", "swelled"),
        ("swam", "swam"),
        ("swung", "swung"),
        ("took", "took"),
        ("taught", "taught"),
        ("tore", "tore"),
        ("told", "told"),
        ("thought", "thought"),
        ("threw", "threw"),
        ("understood", "understood"),
        ("woke", "woke"),
        ("wore", "wore"),
        ("wove", "wove"),
        ("wept", "wept"),
        ("won", "won"),
        ("wound", "wound"),
        ("wrote", "wrote"),
    ]
    .iter()
    .cloned()
    .collect();"""

content = content.replace(old_verb_map, new_verb_map)

# 2. Update is_logically_complete function
old_logical = """/// Check logical completeness: at least 3 words
fn is_logically_complete(text: &str) -> bool {
    let re = Regex::new(r\"\\b\\w+\\b.*\\b\\w+\\b.*\\b\\w+\\b\").unwrap();
    re.is_match(text)
}"""

new_logical = """/// Check logical completeness: at least 2 words
fn is_logically_complete(text: &str) -> bool {
    let re = Regex::new(r\"\\b\\w+\\b.*\\b\\w+\\b\").unwrap();
    re.is_match(text)
}"""

content = content.replace(old_logical, new_logical)

# 3. Update the unit test for logical completeness
old_test = """    #[test]
    fn test_logical_completeness() {
        assert!(is_logically_complete(\"The dog chased the cat\"));
        assert!(is_logically_complete(\"I am here\"));
        assert!(!is_logically_complete(\"Hello world\"));
        assert!(!is_logically_complete(\"Hello\"));
    }"""

new_test = """    #[test]
    fn test_logical_completeness() {
        assert!(is_logically_complete(\"The dog chased the cat\"));
        assert!(is_logically_complete(\"I am here\"));
        assert!(is_logically_complete(\"Hello world\"));  // 2-word sentences now pass
        assert!(!is_logically_complete(\"Hello\"));  // 1-word sentences still fail
    }"""

content = content.replace(old_test, new_test)

# 4. Add newline after #[cfg(test)]
content = content.replace("#[cfg(test)]\nmod tests {", "#[cfg(test)]\n\nmod tests {")

# 5. Update serialize_compressed and deserialize_compressed to handle bincode errors
old_serialize = """#[pyfunction]
#[pyo3(signature = (text, level = 9))]
pub fn serialize_compressed(text: &str, level: i32) -> PyResult<Vec<u8>> {
    let serialized = bincode::serialize(text)?;
    my_compress(&serialized, level)
}

/// Decompress and deserialize data to text
#[pyfunction]
pub fn deserialize_compressed(data: &[u8]) -> PyResult<String> {
    let decompressed = decompress(data)?;
    let deserialized = bincode::deserialize::<String>(&decompressed)?;
    Ok(deserialized)
}"""

new_serialize = """#[pyfunction]
#[pyo3(signature = (text, level = 9))]
pub fn serialize_compressed(text: &str, level: i32) -> PyResult<Vec<u8>> {
    let serialized = bincode::serialize(text).map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
    my_compress(&serialized, level)
}

/// Decompress and deserialize data to text
#[pyfunction]
pub fn deserialize_compressed(data: &[u8]) -> PyResult<String> {
    let decompressed = decompress(data)?;
    let deserialized = bincode::deserialize::<String>(&decompressed)
        .map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(deserialized)
}"""

content = content.replace(old_serialize, new_serialize)

# Write the updated content
with open('src/lib.rs', 'w') as f:
    f.write(content)

print("Generated src/lib.rs with all changes applied.")