"""Check if downloaded PDF contains Chapter 10 geothermal content"""
import asyncio
from compass.utilities.parsing import read_pdf_doc


async def check_pdf():
    path = "downloaded_docs/Colorado/Chaffee/chaffeecountyorgdocumentsdepartmentsplanning2020zoningland20use20codelandusecodepdf.pdf"
    text = await read_pdf_doc(path)
    
    if "chapter 10" in text.lower() and "geothermal" in text.lower():
        print("✓ Found Chapter 10 geothermal content")
        idx = text.lower().find("chapter 10")
        print(f"\nSnippet: {text[idx:idx+300]}...")
    else:
        print("✗ No Chapter 10 geothermal content found")
        if "geothermal" in text.lower():
            print("  (but document does mention geothermal)")
            idx = text.lower().find("geothermal")
            print(f"\nGeothermal snippet: {text[max(0, idx-50):idx+250]}...")


if __name__ == "__main__":
    asyncio.run(check_pdf())
