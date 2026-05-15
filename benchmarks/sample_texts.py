#!/usr/bin/env python3
"""Sample texts for benchmarking rust-cave-001 compression across various domains."""

# Short technical — like the README example (~55 chars)
TECHNICAL_SHORT = "The database needs an index because the queries are too slow."

# Long technical paragraph — real documentation style (~500 chars)
TECHNICAL_LONG = (
    "The system uses a hash-based indexing strategy to optimize query performance. "
    "However, the overhead of maintaining these indexes can be significant when "
    "the underlying data changes frequently. This is because each update operation "
    "must also update the index, which adds latency to write operations. "
    "Therefore, read-heavy workloads benefit most from indexing strategies."
)

# Conversational / interpersonal — slack message style (~300 chars)
CONVERSATIONAL = (
    "Hey team — the deployment went really smoothly today. "
    "The new caching layer was implemented by Sara, and it is working extremely well. "
    "We should probably add some monitoring dashboards though, because we don't "
    "really know how it performs under very heavy load."
)

# Academic / dense writing — abstract style (~400 chars)
ACADEMIC = (
    "The experimental results demonstrate that the proposed methodology significantly "
    "improves compression ratios when compared to traditional approaches. "
    "This improvement was observed consistently across all test datasets, "
    "although the magnitude of improvement varied depending on the characteristics "
    "of the input data. Therefore, the findings suggest that further optimization "
    "of the preprocessing pipeline could yield additional benefits."
)

# Dialogue / casual chat (~250 chars)
DIALOGUE = (
    "So I was like, the database is way too slow for what we need. "
    "And then John said we should just add more indexes, but that seems "
    "like a temporary fix because the real problem is the schema design."
)

# Mixed content — code references, numbers, terms (~300 chars)
MIXED = (
    "The API endpoint at /api/v2/items returns an array of Item objects. "
    "Each object contains an id (integer), name (string), and timestamp "
    "(ISO-8601). However, the response size is extremely large when more "
    "than 1000 items are requested, so pagination is required."
)

# Minimal / already-compressed text
MINIMAL = "Need fast queries now"

# Repetitive / easy text
REPETITIVE = (
    "The quick brown fox jumps over the lazy dog. "
    "The quick brown fox jumps over the lazy dog. "
    "The quick brown fox jumps over the lazy dog. "
    "The quick brown fox jumps over the lazy dog."
)

# Very short sentences
SHORT_SENTENCES = "Fix all bugs. Add new tests. Go home now."

# All texts with labels and descriptions
ALL_TEXTS = {
    "technical_short": {
        "label": "Technical (short)",
        "text": TECHNICAL_SHORT,
    },
    "technical_long": {
        "label": "Technical (paragraph)",
        "text": TECHNICAL_LONG,
    },
    "conversational": {
        "label": "Conversational",
        "text": CONVERSATIONAL,
    },
    "academic": {
        "label": "Academic/Dense",
        "text": ACADEMIC,
    },
    "dialogue": {
        "label": "Dialogue/Chat",
        "text": DIALOGUE,
    },
    "mixed": {
        "label": "Mixed (code+numbers)",
        "text": MIXED,
    },
    "minimal": {
        "label": "Already minimal",
        "text": MINIMAL,
    },
    "repetitive": {
        "label": "Repetitive",
        "text": REPETITIVE,
    },
    "short_sentences": {
        "label": "Short sentences",
        "text": SHORT_SENTENCES,
    },
}
