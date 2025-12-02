<!-- area-id: wanderers-woods-south -->
### Wanderer's Woods (South)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aipom">Aipom</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="aipom" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 8% | 8% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=heracross">Heracross</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="heracross" /> |
| <a href="../pokemon-lookup.html?q=lileep">Lileep</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="lileep" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=paras">Paras</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="paras" /> |
| <a href="../pokemon-lookup.html?q=ralts">Ralts</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="ralts" /> |
| <a href="../pokemon-lookup.html?q=scyther">Scyther</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="scyther" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="wooper" /> |
| <a href="../pokemon-lookup.html?q=anorith">Anorith</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="anorith" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=impidimp">Impidimp</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="impidimp" /> |
| <a href="../pokemon-lookup.html?q=oricorio-sensu">Oricorio Sensu</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="oricorio-sensu" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-south" data-species="paldean-wooper" /> |

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

