<div class="drawer lg:drawer-open">
	<input id="drawer" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content">
		<!-- Page content here -->
		<!-- <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">Open drawer</label> -->
		<slot />
	</div>

	<div class="drawer-side">
		<label for="drawer" class="drawer-overlay" />
		<ul class="menu p-4 w-80 h-full bg-base-200 text-base-content">
			<!-- Sidebar content here -->
			<li><a>Sidebar Item 1</a></li>
			<li><a>Sidebar Item 2</a></li>
		</ul>
	</div>
</div>
