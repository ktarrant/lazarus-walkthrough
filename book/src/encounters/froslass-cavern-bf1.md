<!-- area-id: froslass-cavern-bf1 -->
### Froslass Cavern BF1

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | ☑ |
| --- | --- | --- |
| <a href="../pokemon-lookup.html?q=alolan-vulpix">Alolan Vulpix</a> | 20% | <input type="checkbox" class="caught-check" data-species="alolan-vulpix" /> |
| <a href="../pokemon-lookup.html?q=amaura">Amaura</a> | 2% | <input type="checkbox" class="caught-check" data-species="amaura" /> |
| <a href="../pokemon-lookup.html?q=bronzor">Bronzor</a> | 20% | <input type="checkbox" class="caught-check" data-species="bronzor" /> |
| <a href="../pokemon-lookup.html?q=onix">Onix</a> | 10% | <input type="checkbox" class="caught-check" data-species="onix" /> |
| <a href="../pokemon-lookup.html?q=snorunt">Snorunt</a> | 30% | <input type="checkbox" class="caught-check" data-species="snorunt" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | 10% | <input type="checkbox" class="caught-check" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=swinub">Swinub</a> | 8% | <input type="checkbox" class="caught-check" data-species="swinub" /> |

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

