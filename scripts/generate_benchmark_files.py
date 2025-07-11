#!/usr/bin/env python3
"""
Generate 10000 markdown files with random lorem ipsum content for benchmarking.
"""

import random
from pathlib import Path

# Lorem ipsum word pool
LOREM_WORDS = [
    "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
    "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
    "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
    "exercitation", "ullamco", "laboris", "nisi", "aliquip", "ex", "ea", "commodo",
    "consequat", "duis", "aute", "irure", "in", "reprehenderit", "voluptate",
    "velit", "esse", "cillum", "fugiat", "nulla", "pariatur", "excepteur", "sint",
    "occaecat", "cupidatat", "non", "proident", "sunt", "culpa", "qui", "officia",
    "deserunt", "mollit", "anim", "id", "est", "laborum", "perspiciatis", "unde",
    "omnis", "iste", "natus", "error", "accusantium", "doloremque", "laudantium",
    "totam", "rem", "aperiam", "eaque", "ipsa", "quae", "ab", "illo", "inventore",
    "veritatis", "et", "quasi", "architecto", "beatae", "vitae", "dicta", "explicabo"
]

def generate_sentence(min_words=5, max_words=15):
    """Generate a random sentence with lorem ipsum words."""
    word_count = random.randint(min_words, max_words)
    words = random.choices(LOREM_WORDS, k=word_count)
    sentence = " ".join(words)
    return sentence.capitalize() + "."

def generate_paragraph(min_sentences=3, max_sentences=8):
    """Generate a paragraph with multiple sentences."""
    sentence_count = random.randint(min_sentences, max_sentences)
    sentences = [generate_sentence() for _ in range(sentence_count)]
    return " ".join(sentences)

def generate_heading(level=1):
    """Generate a random heading."""
    word_count = random.randint(2, 6)
    words = random.choices(LOREM_WORDS, k=word_count)
    heading_text = " ".join(word.capitalize() for word in words)
    return "#" * level + " " + heading_text

def generate_list_item():
    """Generate a list item."""
    return "- " + generate_sentence(3, 10)

def generate_markdown_content():
    """Generate random markdown content."""
    content = []
    
    # Generate title and description for front matter
    title_words = random.choices(LOREM_WORDS, k=random.randint(3, 6))
    title = " ".join(word.capitalize() for word in title_words)
    
    description_words = random.choices(LOREM_WORDS, k=random.randint(8, 15))
    description = " ".join(description_words).capitalize() + "."
    
    # Front matter
    content.append("---")
    content.append(f"title: {title}")
    content.append(f"description: {description}")
    content.append("---")
    content.append("")
    
    # Title (using the same title from front matter)
    content.append(f"# {title}")
    content.append("")
    
    # Introduction paragraph
    content.append(generate_paragraph(2, 4))
    content.append("")
    
    # Random sections
    section_count = random.randint(2, 5)
    for _ in range(section_count):
        # Section heading
        content.append(generate_heading(2))
        content.append("")
        
        # Section content
        paragraph_count = random.randint(1, 3)
        for _ in range(paragraph_count):
            content.append(generate_paragraph())
            content.append("")
        
        # Maybe add a list
        if random.random() < 0.4:
            content.append(generate_heading(3))
            content.append("")
            list_items = random.randint(3, 7)
            for _ in range(list_items):
                content.append(generate_list_item())
            content.append("")
        
        # Maybe add a code block
        if random.random() < 0.3:
            content.append("```python")
            content.append("def example_function():")
            content.append("    return \"" + generate_sentence(2, 5) + "\"")
            content.append("```")
            content.append("")
    
    return "\n".join(content)

def main():
    """Generate 10000 markdown files."""
    benchmark_dir = Path("benchmark_docs")
    benchmark_dir.mkdir(exist_ok=True)
    
    print("Generating 10000 markdown files...")
    
    # Create subdirectories to organize files
    subdirs = ["section1", "section2", "section3", "section4", "section5"]
    for subdir in subdirs:
        (benchmark_dir / subdir).mkdir(exist_ok=True)
    
    files_per_dir = 2000
    
    for i in range(10000):
        # Distribute files across subdirectories
        subdir = subdirs[i // files_per_dir]
        file_num = i % files_per_dir
        
        filename = f"doc_{file_num:04d}.md"
        filepath = benchmark_dir / subdir / filename
        
        content = generate_markdown_content()
        
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        if (i + 1) % 1000 == 0:
            print(f"Generated {i + 1} files...")
    
    print(f"Successfully generated 10000 markdown files in {benchmark_dir}/")
    print("Directory structure:")
    for subdir in subdirs:
        file_count = len(list((benchmark_dir / subdir).glob("*.md")))
        print(f"  {subdir}/: {file_count} files")

if __name__ == "__main__":
    main()