<!-- area-id: pntepetal-city -->
### Péntepetal City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=beautifly">Beautifly</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="beautifly" /> |
| <a href="../pokemon-lookup.html?q=comfey">Comfey</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="comfey" /> |
| <a href="../pokemon-lookup.html?q=floette">Floette</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="floette" /> |
| <a href="../pokemon-lookup.html?q=grimer">Grimer</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="grimer" /> |
| <a href="../pokemon-lookup.html?q=kirlia">Kirlia</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="kirlia" /> |
| <a href="../pokemon-lookup.html?q=linoone">Linoone</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="linoone" /> |
| <a href="../pokemon-lookup.html?q=mawile">Mawile</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="mawile" /> |
| <a href="../pokemon-lookup.html?q=alolan-grimer">Alolan Grimer</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="alolan-grimer" /> |
| <a href="../pokemon-lookup.html?q=dustox">Dustox</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="dustox" /> |
| <a href="../pokemon-lookup.html?q=galarian-linoone">Galarian Linoone</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="galarian-linoone" /> |
| <a href="../pokemon-lookup.html?q=pawmo">Pawmo</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="pawmo" /> |
| <a href="../pokemon-lookup.html?q=dratini">Dratini</a> | — | — | 9% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="dratini" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=lapras">Lapras</a> | — | — | 1% | — | — | 5% | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="lapras" /> |
| <a href="../pokemon-lookup.html?q=sealeo">Sealeo</a> | — | — | 60% | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="sealeo" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | — | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=bruxish">Bruxish</a> | — | — | — | — | 20% | 15% | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="bruxish" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=poliwhirl">Poliwhirl</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="pntepetal-city" data-species="poliwhirl" /> |

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

