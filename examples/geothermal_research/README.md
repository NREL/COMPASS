# Geothermal Ordinance Download Script

## Overview
This script downloads geothermal-related ordinance documents for U.S. counties using search engine queries. It's designed to work across all counties without hardcoding specific URLs or domain patterns.

## How It Works
1. **Constructs county domain**: Automatically generates likely county website domain (e.g., `chaffeecounty.org`)
2. **Multiple search strategies**:
   - Site-specific searches targeting county domains
   - 1041 regulations (Colorado's "Areas and Activities of State Interest")
   - General geothermal ordinance/regulation/chapter searches
3. **Downloads and organizes**: Saves PDFs to `downloaded_docs/State/County/` structure

## Search Query Strategy
The script uses 21 different search query templates targeting:
- 1041 regulations and state interest areas
- Geothermal chapters and regulations
- Land use codes and zoning documents
- Development standards and permit regulations

## Limitations
**Search engines may not index all regulatory documents** because:
- PDFs with timestamp/session parameters (e.g., `document.pdf?t=timestamp`) often aren't indexed
- Recently updated documents may not be crawled yet
- Chapter-specific PDFs behind navigation pages may be missed

## Expected Results
The script will download a mix of documents:
- ✅ General land use codes and zoning documents
- ✅ State geological surveys and background research
- ✅ Some regulatory chapters (if indexed by search engines)
- ❌ May miss specific chapter PDFs with dynamic URLs

## For Production Use
To achieve >90% coverage across U.S. counties, the download strategy should be combined with:
1. **This search-based approach** (baseline coverage)
2. **HTML page parsing** of county planning/zoning pages to extract PDF links
3. **Document classification** in the extraction pipeline to identify actual regulations vs background material

## Usage
```bash
pixi run -e default python download_docs.py
```

Edit the `jurisdiction` variable in the script to test different counties.

## Example: Chaffee County, Colorado
The script successfully downloads:
- 2014 Land Use Code (indexed)
- Comprehensive planning documents
- Geological survey reports
- Various geothermal research PDFs

May miss:
- Chapter 10 (53007.pdf) if it has URL parameters preventing indexing

The extraction/validation pipeline should handle document filtering to identify which PDFs contain actual ordinance text with technical requirements (setbacks, noise limits, etc.).
