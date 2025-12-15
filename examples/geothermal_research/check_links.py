import requests
from bs4 import BeautifulSoup

url = 'https://www.chaffeecounty.org/departments/community_planning_natural_resources/land_use_code.php'
response = requests.get(url)
soup = BeautifulSoup(response.text, 'html.parser')

print("Links containing 'chapter' or 'geothermal':")
for link in soup.find_all('a', href=True):
    text = link.get_text().strip()
    href = link['href']
    if 'chapter' in text.lower() or 'geothermal' in text.lower() or 'chapter' in href.lower():
        print(f"{text}: {href}")
