<!-- area-id: riverwalk-trail-south -->
### Riverwalk Trail (South)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bounsweet">Bounsweet</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="bounsweet" /> |
| <a href="../pokemon-lookup.html?q=buizel">Buizel</a> | 20% | 20% | — | — | 40% | — | <input type="checkbox" class="caught-check" data-species="buizel" /> |
| <a href="../pokemon-lookup.html?q=cufant">Cufant</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cufant" /> |
| <a href="../pokemon-lookup.html?q=ekans">Ekans</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="ekans" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="paldean-wooper" /> |
| <a href="../pokemon-lookup.html?q=silcoon">Silcoon</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="silcoon" /> |
| <a href="../pokemon-lookup.html?q=tropius">Tropius</a> | 2% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="tropius" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wooper" /> |
| <a href="../pokemon-lookup.html?q=wurmple">Wurmple</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wurmple" /> |
| <a href="../pokemon-lookup.html?q=zigzagoon">Zigzagoon</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=cascoon">Cascoon</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cascoon" /> |
| <a href="../pokemon-lookup.html?q=dreepy">Dreepy</a> | — | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dreepy" /> |
| <a href="../pokemon-lookup.html?q=galarian-zigzagoon">Galarian Zigzagoon</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="galarian-zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=pumpkaboo">Pumpkaboo</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pumpkaboo" /> |
| <a href="../pokemon-lookup.html?q=barboach">Barboach</a> | — | — | 10% | — | — | 40% | <input type="checkbox" class="caught-check" data-species="barboach" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | 60% | — | 60% | — | <input type="checkbox" class="caught-check" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | 30% | 30% | — | — | <input type="checkbox" class="caught-check" data-species="skrelp" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=whiscash">Whiscash</a> | — | — | — | — | — | 20% | <input type="checkbox" class="caught-check" data-species="whiscash" /> |

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

