<!-- area-id: pollen-road -->
### Pollen Road

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=braviary">Braviary</a> | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="braviary" /> |
| <a href="../pokemon-lookup.html?q=comfey">Comfey</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="comfey" /> |
| <a href="../pokemon-lookup.html?q=dodrio">Dodrio</a> | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dodrio" /> |
| <a href="../pokemon-lookup.html?q=floette-red-flower">Floette Red Flower</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="floette-red-flower" /> |
| <a href="../pokemon-lookup.html?q=hawlucha">Hawlucha</a> | 8% | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hawlucha" /> |
| <a href="../pokemon-lookup.html?q=hisuian-electrode">Hisuian Electrode</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hisuian-electrode" /> |
| <a href="../pokemon-lookup.html?q=pachirisu">Pachirisu</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pachirisu" /> |
| <a href="../pokemon-lookup.html?q=pikachu">Pikachu</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pikachu" /> |
| <a href="../pokemon-lookup.html?q=skiploom">Skiploom</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="skiploom" /> |
| <a href="../pokemon-lookup.html?q=yanmega">Yanmega</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="yanmega" /> |
| <a href="../pokemon-lookup.html?q=charjabug">Charjabug</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="charjabug" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=espathra">Espathra</a> | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="espathra" /> |
| <a href="../pokemon-lookup.html?q=floette-yellow-flower">Floette Yellow Flower</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="floette-yellow-flower" /> |
| <a href="../pokemon-lookup.html?q=hisuian-braviary">Hisuian Braviary</a> | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hisuian-braviary" /> |
| <a href="../pokemon-lookup.html?q=togedemaru">Togedemaru</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="togedemaru" /> |
| <a href="../pokemon-lookup.html?q=dhelmise">Dhelmise</a> | — | — | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="dhelmise" /> |
| <a href="../pokemon-lookup.html?q=dragonair">Dragonair</a> | — | — | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="dragonair" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=swanna">Swanna</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="swanna" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | 30% | 20% | — | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=cloyster">Cloyster</a> | — | — | — | — | — | 15% | <input type="checkbox" class="caught-check" data-species="cloyster" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=huntail">Huntail</a> | — | — | — | — | — | 5% | <input type="checkbox" class="caught-check" data-species="huntail" /> |
| <a href="../pokemon-lookup.html?q=octillery">Octillery</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="octillery" /> |

<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>

