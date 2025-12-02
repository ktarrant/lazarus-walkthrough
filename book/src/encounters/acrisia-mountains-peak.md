<!-- area-id: acrisia-mountains-peak -->
### Acrisia Mountains (Peak)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=flabebe-blue-flower">Flabebe Blue Flower</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flabebe-blue-flower" /> |
| <a href="../pokemon-lookup.html?q=flabebe-orange-flower">Flabebe Orange Flower</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flabebe-orange-flower" /> |
| <a href="../pokemon-lookup.html?q=flabebe-red-flower">Flabebe Red Flower</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flabebe-red-flower" /> |
| <a href="../pokemon-lookup.html?q=flabebe-white-flower">Flabebe White Flower</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flabebe-white-flower" /> |
| <a href="../pokemon-lookup.html?q=flabebe-yellow-flower">Flabebe Yellow Flower</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flabebe-yellow-flower" /> |
| <a href="../pokemon-lookup.html?q=flittle">Flittle</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="flittle" /> |
| <a href="../pokemon-lookup.html?q=pichu">Pichu</a> | 20% | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="pichu" /> |
| <a href="../pokemon-lookup.html?q=snover">Snover</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="snover" /> |
| <a href="../pokemon-lookup.html?q=bronzor">Bronzor</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="bronzor" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=rockruff">Rockruff</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="rockruff" /> |
| <a href="../pokemon-lookup.html?q=skiddo">Skiddo</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="skiddo" /> |
| <a href="../pokemon-lookup.html?q=spinarak">Spinarak</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="acrisia-mountains-peak" data-species="spinarak" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

