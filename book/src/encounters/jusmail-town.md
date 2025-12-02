<!-- area-id: jusmail-town -->
### Jusmail Town

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bounsweet">Bounsweet</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="bounsweet" /> |
| <a href="../pokemon-lookup.html?q=cutiefly">Cutiefly</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="cutiefly" /> |
| <a href="../pokemon-lookup.html?q=eevee">Eevee</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="eevee" /> |
| <a href="../pokemon-lookup.html?q=growlithe">Growlithe</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="growlithe" /> |
| <a href="../pokemon-lookup.html?q=hisuian-growlithe">Hisuian Growlithe</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="hisuian-growlithe" /> |
| <a href="../pokemon-lookup.html?q=oricorio-baile">Oricorio Baile</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="oricorio-baile" /> |
| <a href="../pokemon-lookup.html?q=pikipek">Pikipek</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="pikipek" /> |
| <a href="../pokemon-lookup.html?q=skiddo">Skiddo</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="skiddo" /> |
| <a href="../pokemon-lookup.html?q=wurmple">Wurmple</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="wurmple" /> |
| <a href="../pokemon-lookup.html?q=alolan-vulpix">Alolan Vulpix</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="alolan-vulpix" /> |
| <a href="../pokemon-lookup.html?q=hisuian-voltorb">Hisuian Voltorb</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="hisuian-voltorb" /> |
| <a href="../pokemon-lookup.html?q=ralts">Ralts</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="ralts" /> |
| <a href="../pokemon-lookup.html?q=voltorb">Voltorb</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="voltorb" /> |
| <a href="../pokemon-lookup.html?q=vulpix">Vulpix</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="vulpix" /> |
| <a href="../pokemon-lookup.html?q=woobat">Woobat</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="jusmail-town" data-species="woobat" /> |

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

