<!-- area-id: acrisia-mountains-peak -->
### Acrisia Mountains (Peak)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=flabebe">Flabebe Blue Flower</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="flabebe" /> |
| <a href="../pokemon-lookup.html?q=flabebe">Flabebe Orange Flower</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="flabebe" /> |
| <a href="../pokemon-lookup.html?q=flabebe">Flabebe Red Flower</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="flabebe" /> |
| <a href="../pokemon-lookup.html?q=flabebe">Flabebe White Flower</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="flabebe" /> |
| <a href="../pokemon-lookup.html?q=flabebe">Flabebe Yellow Flower</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="flabebe" /> |
| <a href="../pokemon-lookup.html?q=flittle">Flittle</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="flittle" /> |
| <a href="../pokemon-lookup.html?q=pichu">Pichu</a> | 20% | 10% | <input type="checkbox" class="caught-check" data-species="pichu" /> |
| <a href="../pokemon-lookup.html?q=snover">Snover</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="snover" /> |
| <a href="../pokemon-lookup.html?q=bronzor">Bronzor</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="bronzor" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=rockruff">Rockruff</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="rockruff" /> |
| <a href="../pokemon-lookup.html?q=skiddo">Skiddo</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="skiddo" /> |
| <a href="../pokemon-lookup.html?q=spinarak">Spinarak</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="spinarak" /> |

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

