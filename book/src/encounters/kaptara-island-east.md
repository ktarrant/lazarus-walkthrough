<!-- area-id: kaptara-island-east -->
### Kaptara Island (East)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=charjabug">Charjabug</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="charjabug" /> |
| <a href="../pokemon-lookup.html?q=cherrim">Cherrim</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="cherrim" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=doublade">Doublade</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="doublade" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=perrserker">Perrserker</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="perrserker" /> |
| <a href="../pokemon-lookup.html?q=skiploom">Skiploom</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="skiploom" /> |
| <a href="../pokemon-lookup.html?q=sligoo">Sligoo</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="sligoo" /> |
| <a href="../pokemon-lookup.html?q=tauros">Tauros</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="tauros" /> |
| <a href="../pokemon-lookup.html?q=tauros-combat-breed">Tauros Combat Breed</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="tauros-combat-breed" /> |
| <a href="../pokemon-lookup.html?q=weepinbell">Weepinbell</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="weepinbell" /> |
| <a href="../pokemon-lookup.html?q=arbok">Arbok</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="arbok" /> |
| <a href="../pokemon-lookup.html?q=hisuian-sligoo">Hisuian Sligoo</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="hisuian-sligoo" /> |
| <a href="../pokemon-lookup.html?q=skuntank">Skuntank</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="skuntank" /> |
| <a href="../pokemon-lookup.html?q=tauros-aqua-breed">Tauros Aqua Breed</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="tauros-aqua-breed" /> |
| <a href="../pokemon-lookup.html?q=tauros-blaze-breed">Tauros Blaze Breed</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="tauros-blaze-breed" /> |
| <a href="../pokemon-lookup.html?q=togedemaru">Togedemaru</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-east" data-species="togedemaru" /> |

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

