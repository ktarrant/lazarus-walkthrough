<!-- area-id: froslass-cavern-bf2 -->
### Froslass Cavern BF2

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=amaura">Amaura</a> | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="amaura" /> |
| <a href="../pokemon-lookup.html?q=bronzor">Bronzor</a> | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="bronzor" /> |
| <a href="../pokemon-lookup.html?q=cubchoo">Cubchoo</a> | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cubchoo" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=snom">Snom</a> | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="snom" /> |
| <a href="../pokemon-lookup.html?q=snorunt">Snorunt</a> | 30% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="snorunt" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | 10% | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=swinub">Swinub</a> | 13% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="swinub" /> |
| <a href="../pokemon-lookup.html?q=lapras">Lapras</a> | — | 1% | — | — | — | <input type="checkbox" class="caught-check" data-species="lapras" /> |
| <a href="../pokemon-lookup.html?q=sealeo">Sealeo</a> | — | 9% | — | — | — | <input type="checkbox" class="caught-check" data-species="sealeo" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | 60% | 30% | 40% | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=poliwhirl">Poliwhirl</a> | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="poliwhirl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |

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

