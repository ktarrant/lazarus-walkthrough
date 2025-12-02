<!-- area-id: davosi-straits -->
### Davosi Straits

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=dhelmise">Dhelmise</a> | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="dhelmise" /> |
| <a href="../pokemon-lookup.html?q=dragonair">Dragonair</a> | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="dragonair" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=swanna">Swanna</a> | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="swanna" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | 30% | 20% | — | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=cloyster">Cloyster</a> | — | — | — | 15% | <input type="checkbox" class="caught-check" data-species="cloyster" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=gorebyss">Gorebyss</a> | — | — | — | 5% | <input type="checkbox" class="caught-check" data-species="gorebyss" /> |
| <a href="../pokemon-lookup.html?q=octillery">Octillery</a> | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="octillery" /> |

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

